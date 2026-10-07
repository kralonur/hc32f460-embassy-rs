#[doc = "Register `TDR` reader"]
pub type R = crate::R<TdrSpec>;
#[doc = "Register `TDR` writer"]
pub type W = crate::W<TdrSpec>;
#[doc = "Field `TDR` reader - desc TDR"]
pub type TdrR = crate::FieldReader<u16>;
#[doc = "Field `TDR` writer - desc TDR"]
pub type TdrW<'a, REG> = crate::FieldWriter<'a, REG, 9, u16>;
#[doc = "Field `MPID` reader - desc MPID"]
pub type MpidR = crate::BitReader;
#[doc = "Field `MPID` writer - desc MPID"]
pub type MpidW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:8 - desc TDR"]
    #[inline(always)]
    pub fn tdr(&self) -> TdrR {
        TdrR::new(self.bits & 0x01ff)
    }
    #[doc = "Bit 9 - desc MPID"]
    #[inline(always)]
    pub fn mpid(&self) -> MpidR {
        MpidR::new(((self.bits >> 9) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:8 - desc TDR"]
    #[inline(always)]
    pub fn tdr(&mut self) -> TdrW<'_, TdrSpec> {
        TdrW::new(self, 0)
    }
    #[doc = "Bit 9 - desc MPID"]
    #[inline(always)]
    pub fn mpid(&mut self) -> MpidW<'_, TdrSpec> {
        MpidW::new(self, 9)
    }
}
#[doc = "desc TDR\n\nYou can [`read`](crate::Reg::read) this register and get [`tdr::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tdr::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TdrSpec;
impl crate::RegisterSpec for TdrSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`tdr::R`](R) reader structure"]
impl crate::Readable for TdrSpec {}
#[doc = "`write(|w| ..)` method takes [`tdr::W`](W) writer structure"]
impl crate::Writable for TdrSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TDR to value 0x01ff"]
impl crate::Resettable for TdrSpec {
    const RESET_VALUE: u16 = 0x01ff;
}
