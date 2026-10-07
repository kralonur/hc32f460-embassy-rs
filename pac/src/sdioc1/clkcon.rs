#[doc = "Register `CLKCON` reader"]
pub type R = crate::R<ClkconSpec>;
#[doc = "Register `CLKCON` writer"]
pub type W = crate::W<ClkconSpec>;
#[doc = "Field `ICE` reader - desc ICE"]
pub type IceR = crate::BitReader;
#[doc = "Field `ICE` writer - desc ICE"]
pub type IceW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CE` reader - desc CE"]
pub type CeR = crate::BitReader;
#[doc = "Field `CE` writer - desc CE"]
pub type CeW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FS` reader - desc FS"]
pub type FsR = crate::FieldReader;
#[doc = "Field `FS` writer - desc FS"]
pub type FsW<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - desc ICE"]
    #[inline(always)]
    pub fn ice(&self) -> IceR {
        IceR::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 2 - desc CE"]
    #[inline(always)]
    pub fn ce(&self) -> CeR {
        CeR::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 8:15 - desc FS"]
    #[inline(always)]
    pub fn fs(&self) -> FsR {
        FsR::new(((self.bits >> 8) & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 0 - desc ICE"]
    #[inline(always)]
    pub fn ice(&mut self) -> IceW<'_, ClkconSpec> {
        IceW::new(self, 0)
    }
    #[doc = "Bit 2 - desc CE"]
    #[inline(always)]
    pub fn ce(&mut self) -> CeW<'_, ClkconSpec> {
        CeW::new(self, 2)
    }
    #[doc = "Bits 8:15 - desc FS"]
    #[inline(always)]
    pub fn fs(&mut self) -> FsW<'_, ClkconSpec> {
        FsW::new(self, 8)
    }
}
#[doc = "desc CLKCON\n\nYou can [`read`](crate::Reg::read) this register and get [`clkcon::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`clkcon::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ClkconSpec;
impl crate::RegisterSpec for ClkconSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`clkcon::R`](R) reader structure"]
impl crate::Readable for ClkconSpec {}
#[doc = "`write(|w| ..)` method takes [`clkcon::W`](W) writer structure"]
impl crate::Writable for ClkconSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CLKCON to value 0x02"]
impl crate::Resettable for ClkconSpec {
    const RESET_VALUE: u16 = 0x02;
}
