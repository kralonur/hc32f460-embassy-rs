#[doc = "Register `PORRC` reader"]
pub type R = crate::R<PorrcSpec>;
#[doc = "Register `PORRC` writer"]
pub type W = crate::W<PorrcSpec>;
#[doc = "Field `POR13` reader - desc POR13"]
pub type Por13R = crate::BitReader;
#[doc = "Field `POR13` writer - desc POR13"]
pub type Por13W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POR14` reader - desc POR14"]
pub type Por14R = crate::BitReader;
#[doc = "Field `POR14` writer - desc POR14"]
pub type Por14W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `POR15` reader - desc POR15"]
pub type Por15R = crate::BitReader;
#[doc = "Field `POR15` writer - desc POR15"]
pub type Por15W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 13 - desc POR13"]
    #[inline(always)]
    pub fn por13(&self) -> Por13R {
        Por13R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - desc POR14"]
    #[inline(always)]
    pub fn por14(&self) -> Por14R {
        Por14R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - desc POR15"]
    #[inline(always)]
    pub fn por15(&self) -> Por15R {
        Por15R::new(((self.bits >> 15) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 13 - desc POR13"]
    #[inline(always)]
    pub fn por13(&mut self) -> Por13W<'_, PorrcSpec> {
        Por13W::new(self, 13)
    }
    #[doc = "Bit 14 - desc POR14"]
    #[inline(always)]
    pub fn por14(&mut self) -> Por14W<'_, PorrcSpec> {
        Por14W::new(self, 14)
    }
    #[doc = "Bit 15 - desc POR15"]
    #[inline(always)]
    pub fn por15(&mut self) -> Por15W<'_, PorrcSpec> {
        Por15W::new(self, 15)
    }
}
#[doc = "desc PORRC\n\nYou can [`read`](crate::Reg::read) this register and get [`porrc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`porrc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PorrcSpec;
impl crate::RegisterSpec for PorrcSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`porrc::R`](R) reader structure"]
impl crate::Readable for PorrcSpec {}
#[doc = "`write(|w| ..)` method takes [`porrc::W`](W) writer structure"]
impl crate::Writable for PorrcSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PORRC to value 0"]
impl crate::Resettable for PorrcSpec {}
