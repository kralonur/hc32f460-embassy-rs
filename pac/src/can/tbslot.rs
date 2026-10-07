#[doc = "Register `TBSLOT` reader"]
pub type R = crate::R<TbslotSpec>;
#[doc = "Register `TBSLOT` writer"]
pub type W = crate::W<TbslotSpec>;
#[doc = "Field `TBPTR` reader - desc TBPTR"]
pub type TbptrR = crate::FieldReader;
#[doc = "Field `TBPTR` writer - desc TBPTR"]
pub type TbptrW<'a, REG> = crate::FieldWriter<'a, REG, 6>;
#[doc = "Field `TBF` reader - desc TBF"]
pub type TbfR = crate::BitReader;
#[doc = "Field `TBF` writer - desc TBF"]
pub type TbfW<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TBE` reader - desc TBE"]
pub type TbeR = crate::BitReader;
#[doc = "Field `TBE` writer - desc TBE"]
pub type TbeW<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:5 - desc TBPTR"]
    #[inline(always)]
    pub fn tbptr(&self) -> TbptrR {
        TbptrR::new(self.bits & 0x3f)
    }
    #[doc = "Bit 6 - desc TBF"]
    #[inline(always)]
    pub fn tbf(&self) -> TbfR {
        TbfR::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - desc TBE"]
    #[inline(always)]
    pub fn tbe(&self) -> TbeR {
        TbeR::new(((self.bits >> 7) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 0:5 - desc TBPTR"]
    #[inline(always)]
    pub fn tbptr(&mut self) -> TbptrW<'_, TbslotSpec> {
        TbptrW::new(self, 0)
    }
    #[doc = "Bit 6 - desc TBF"]
    #[inline(always)]
    pub fn tbf(&mut self) -> TbfW<'_, TbslotSpec> {
        TbfW::new(self, 6)
    }
    #[doc = "Bit 7 - desc TBE"]
    #[inline(always)]
    pub fn tbe(&mut self) -> TbeW<'_, TbslotSpec> {
        TbeW::new(self, 7)
    }
}
#[doc = "desc TBSLOT\n\nYou can [`read`](crate::Reg::read) this register and get [`tbslot::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tbslot::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TbslotSpec;
impl crate::RegisterSpec for TbslotSpec {
    type Ux = u8;
}
#[doc = "`read()` method returns [`tbslot::R`](R) reader structure"]
impl crate::Readable for TbslotSpec {}
#[doc = "`write(|w| ..)` method takes [`tbslot::W`](W) writer structure"]
impl crate::Writable for TbslotSpec {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TBSLOT to value 0"]
impl crate::Resettable for TbslotSpec {}
