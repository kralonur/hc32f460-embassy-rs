#[doc = "Register `DCOM` reader"]
pub type R = crate::R<DcomSpec>;
#[doc = "Register `DCOM` writer"]
pub type W = crate::W<DcomSpec>;
#[doc = "Field `DCOM` reader - desc DCOM"]
pub type DcomR = crate::FieldReader;
#[doc = "Field `DCOM` writer - desc DCOM"]
pub type DcomW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - desc DCOM"]
    #[inline(always)]
    pub fn dcom(&self) -> DcomR {
        DcomR::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - desc DCOM"]
    #[inline(always)]
    pub fn dcom(&mut self) -> DcomW<'_, DcomSpec> {
        DcomW::new(self, 0)
    }
}
#[doc = "desc DCOM\n\nYou can [`read`](crate::Reg::read) this register and get [`dcom::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`dcom::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DcomSpec;
impl crate::RegisterSpec for DcomSpec {
    type Ux = u32;
}
#[doc = "`read()` method returns [`dcom::R`](R) reader structure"]
impl crate::Readable for DcomSpec {}
#[doc = "`write(|w| ..)` method takes [`dcom::W`](W) writer structure"]
impl crate::Writable for DcomSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DCOM to value 0"]
impl crate::Resettable for DcomSpec {}
